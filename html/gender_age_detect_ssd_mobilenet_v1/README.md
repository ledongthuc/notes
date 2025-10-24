# Camera Capture React App

A simple React application that captures the user's camera and displays it in the browser.

## Features

- Full-screen camera display
- Automatic camera access on page load
- Error handling for camera permissions
- Responsive design
- No controls - just camera feed

## Getting Started

### Prerequisites

- Node.js (version 14 or higher)
- npm or yarn

### Installation

1. Install dependencies:
```bash
npm install
```

2. Start the development server:
```bash
npm start
```

3. Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

## Project Structure

```
src/
├── components/
│   ├── Camera.js          # Main camera component
│   └── Camera.css         # Camera component styles
├── App.js                 # Main app component
├── App.css               # App styles
└── index.js              # Entry point
```

## Usage

The app will automatically request camera access when loaded. Grant permission to see your camera feed in full screen.

## Browser Compatibility

- Chrome/Chromium browsers
- Firefox
- Safari (with getUserMedia support)
- Edge

## Notes

- Camera permissions are required
- HTTPS is recommended for production deployment
- The app uses the front-facing camera by default
