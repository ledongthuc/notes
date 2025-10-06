import React, { useState, useRef, useEffect } from 'react';
import * as faceapi from 'face-api.js';
import * as webllm from "@mlc-ai/web-llm";
import './Camera.css';

// Global model loading state to ensure models are loaded only once
let globalModelsLoaded = false;
let globalModelLoadingPromise = null;

 // Initialize with a progress callback
 const initProgressCallback = (progress) => {
  console.log("Model loading progress:", progress);
};

const engine = await webllm.CreateMLCEngine("Llama-3.2-3B-Instruct-q4f32_1-MLC", { initProgressCallback });
const engineInstance = new webllm.MLCEngine({ initProgressCallback });
await engineInstance.reload("Llama-3.2-3B-Instruct-q4f32_1-MLC");

const Camera = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [modelsLoaded, setModelsLoaded] = useState(globalModelsLoaded);
  const [detections, setDetections] = useState([]);
  const [llmLoading, setLlmLoading] = useState(false);
  const [llmProgress, setLlmProgress] = useState(0);
  const [llmResponse, setLlmResponse] = useState('');
  const videoRef = useRef(null);
  const canvasRef = useRef(null);
  const detectionIntervalRef = useRef(null);
  const llmIntervalRef = useRef(null);
  const detectionsRef = useRef([]);
  const llmRunningRef = useRef(false);

  useEffect(() => {
    // Only load models if they haven't been loaded globally
    if (!globalModelsLoaded && !globalModelLoadingPromise) {
      globalModelLoadingPromise = loadModels();
    } else if (globalModelsLoaded) {
      // Models already loaded, start camera and detection
      setModelsLoaded(true);
      startCamera();
      startFaceDetection();
    } else if (globalModelLoadingPromise) {
      // Models are currently loading, wait for them
      globalModelLoadingPromise.then(() => {
        setModelsLoaded(true);
        startCamera();
        startFaceDetection();
      });
    }
  }, []);


  const loadModels = async () => {
    try {
      setLoading(true);
      setError('');
      
      console.log('Starting to load models...');
      
      // Load tiny face detector model (lighter and faster)
      console.log('Loading tiny face detector...');
      await faceapi.nets.tinyFaceDetector.loadFromUri('/models');
      console.log('Tiny face detector loaded successfully');
      
      // Load age/gender model  
      console.log('Loading age/gender model...');
      await faceapi.nets.ageGenderNet.loadFromUri('/models');
      console.log('Age/gender model loaded successfully');
      
      // Set global state to prevent future loads
      globalModelsLoaded = true;
      globalModelLoadingPromise = null;
      
      setModelsLoaded(true);
      setLoading(false);
      
      // Start camera after models are loaded
      startCamera();
      
      // Start face detection
      startFaceDetection();
    } catch (err) {
      console.error('Error loading models:', err);
      globalModelLoadingPromise = null; // Reset promise on error
      setLoading(false);
      setError('Failed to load face detection models.');
    }
  };

  const startCamera = async () => {
    try {
      setLoading(true);
      setError('');
      
      // Request camera access
      const stream = await navigator.mediaDevices.getUserMedia({
        video: {
          width: { ideal: 1280 },
          height: { ideal: 720 },
          facingMode: 'user' // Front camera
        },
        audio: false
      });
      
      // Set the video source to the camera stream
      if (videoRef.current) {
        videoRef.current.srcObject = stream;
        
        // Handle video load
        videoRef.current.onloadedmetadata = () => {
          videoRef.current.play();
          setLoading(false);
        };
      }
      
    } catch (err) {
      console.error('Error accessing camera:', err);
      setLoading(false);
      setError('Unable to access camera. Please ensure you have granted camera permissions.');
    }
  };

  const startFaceDetection = () => {
    if (detectionIntervalRef.current) {
      clearInterval(detectionIntervalRef.current);
    }

    if (llmIntervalRef.current) {
      clearInterval(llmIntervalRef.current);
    }

    detectionIntervalRef.current = setInterval(async () => {
      if (videoRef.current) {
        await detectFaces();
      }
    }, 100); // Detect faces every 100ms

    // Start LLM response generation interval
    llmIntervalRef.current = setInterval(async () => {
      // Only call if not already running
      if (!llmRunningRef.current) {
        await generateLLMResponseFromDetections(detectionsRef.current);
      }
    }, 10000); // Generate LLM Response every 10 seconds
  };

  const detectFaces = async () => {
    try {
      if (!videoRef.current) return;

      const detections = await faceapi
        .detectAllFaces(videoRef.current, new faceapi.TinyFaceDetectorOptions())
        .withAgeAndGender();

      setDetections(detections);
      detectionsRef.current = detections; // Update the ref with current detections
      drawDetections(detections);
    } catch (err) {
      console.error('Error detecting faces:', err);
    }
  };

  const generateLLMResponseFromDetections = async (detections) => {
    // Check if already running, if so, skip this execution
    if (llmRunningRef.current) {
      console.log("LLM response generation already in progress, skipping...");
      return;
    }

    // Set running flag to true
    llmRunningRef.current = true;
    
    try {
      console.log("Generating LLM response from detections...", detections.length);
      if (!detections || detections.length === 0) return;

      // Start loading state
      setLlmLoading(true);
      setLlmProgress(0);
      setLlmResponse('');

      // Simulate progress updates
      const progressInterval = setInterval(() => {
        setLlmProgress(prev => {
          if (prev >= 90) return prev;
          return prev + Math.random() * 10;
        });
      }, 200);

      const number_of_detections = detections.length;
      let composed_response = "";
      for (let index = 0; index < detections.length; index++) {
        const detection = detections[index];
        composed_response += (index + 1) + " person is a " + detection.gender + " with an age of " + detection.age + 10 + " years old, ";
      }
      const messages = [
        { role: "user", content: "Say hello group of " + number_of_detections + " customers." }
      ];
      
      const reply = await engine.chat.completions.create({
        messages,
      });
      
      // Clear progress interval and set final progress
      clearInterval(progressInterval);
      setLlmProgress(100);
      
      // Set the response content
      const responseContent = reply.choices[0].message.content;
      setLlmResponse(responseContent);
      
      console.log(responseContent);
      console.log(reply.usage);
    } catch (error) {
      console.error("Error generating LLM response:", error);
      setLlmResponse("Error generating response: " + error.message);
    } finally {
      // Always reset the running flag and loading state when done
      llmRunningRef.current = false;
      setLlmLoading(false);
    }
  };

  const drawDetections = (detections) => {
    if (!canvasRef.current || !videoRef.current) return;

    const canvas = canvasRef.current;
    const video = videoRef.current;
    const ctx = canvas.getContext('2d');

    // Set canvas size to match video
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Draw detections
    detections.forEach((detection) => {
      const { x, y, width, height } = detection.detection.box;
      
      // Draw bounding box
      ctx.strokeStyle = '#00ff00';
      ctx.lineWidth = 2;
      ctx.strokeRect(x, y, width, height);

      // Draw age and gender info
      if (detection.age && detection.gender) {
        const age = Math.round(detection.age);
        const gender = detection.gender;
        const confidence = Math.round(detection.genderProbability * 100);

        ctx.fillStyle = '#00ff00';
        ctx.font = '16px Arial';
        ctx.fillText(`${gender} (${confidence}%)`, x, y - 10);
        ctx.fillText(`Age: ${age}`, x, y - 30);
      }
    });
  };

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (detectionIntervalRef.current) {
        clearInterval(detectionIntervalRef.current);
      }
      if (llmIntervalRef.current) {
        clearInterval(llmIntervalRef.current);
      }
    };
  }, []);

  return (
    <div className="camera-container">
      <video 
        id="video"
        ref={videoRef}
        autoPlay 
        muted 
        playsInline
        className="camera-video"
      />
      
      <canvas
        ref={canvasRef}
        className="detection-canvas"
      />
      
      {loading && (
        <div className="loading">
          {modelsLoaded ? 'Loading camera...' : 'Loading face detection models...'}
        </div>
      )}
      
      {error && (
        <div className="error">
          {error}
        </div>
      )}

      {detections.length > 0 && (
        <div className="detection-info">
          <h3>Detected Faces: {detections.length}</h3>
          {detections.map((detection, index) => (
            <div key={index} className="face-info">
              <p>Age: {Math.round(detection.age)}</p>
              <p>Gender: {detection.gender} ({Math.round(detection.genderProbability * 100)}%)</p>
            </div>
          ))}
        </div>
      )}

      {/* LLM Loading and Response Box */}
      {(llmLoading || llmResponse) && (
        <div className="llm-box">
          {llmLoading && !llmResponse && (
            <div className="llm-loading">
              <h4>🤖 AI is thinking...</h4>
              <div className="progress-bar">
                <div 
                  className="progress-fill" 
                  style={{ width: `${llmProgress}%` }}
                ></div>
              </div>
              <p className="progress-text">{Math.round(llmProgress)}%</p>
            </div>
          )}
          {llmResponse && !llmLoading && (
            <div className="llm-response">
              <h4>🤖 AI Response:</h4>
              <p>{llmResponse}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default Camera;
