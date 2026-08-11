import React, { useState, useRef, useEffect } from 'react';
import * as faceapi from 'face-api.js';
import './Camera.css';

// Global model loading state to ensure models are loaded only once
let globalModelsLoaded = false;
let globalModelLoadingPromise = null;

 // Initialize with a progress callback
 const initProgressCallback = (progress) => {
  console.log("Model loading progress:", progress);
};

const Camera = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [modelsLoaded, setModelsLoaded] = useState(globalModelsLoaded);
  const [detections, setDetections] = useState([]);
  const videoRef = useRef(null);
  const canvasRef = useRef(null);
  const detectionIntervalRef = useRef(null);
  const detectionsRef = useRef([]);

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
      
      // Load  detector model (lighter and faster)
      console.log('Loading SSD Mobilenetv1 face detector...');
      await faceapi.nets.ssdMobilenetv1.loadFromUri('/models');
      await faceapi.nets.faceLandmark68Net.loadFromUri('/models');
      await faceapi.nets.faceRecognitionNet.loadFromUri('/models');
      await faceapi.nets.ageGenderNet.loadFromUri('/models');
      console.log('SSD Mobilenetv1 face detector loaded successfully');
      
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

    detectionIntervalRef.current = setInterval(async () => {
      if (videoRef.current) {
        await detectFaces();
      }
    }, 100); // Detect faces every 100ms
  };

  const detectFaces = async () => {
    try {
      if (!videoRef.current) return;

      const detections = await faceapi
        .detectAllFaces(videoRef.current, new faceapi.SsdMobilenetv1Options())
        .withFaceLandmarks()
        .withAgeAndGender();

      setDetections(detections);
      detectionsRef.current = detections; // Update the ref with current detections
      drawDetections(detections);
    } catch (err) {
      console.error('Error detecting faces:', err);
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

    </div>
  );
};

export default Camera;
