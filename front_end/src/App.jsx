import { useState } from 'react'
import Webcam from 'react-webcam';
import './App.css'

function App() {

  //const convertImage = (image

  const binEncode = (data) => {
    var binArray = []
    var datEncode = "";

    for (let i=0; i < data.length; i++) {
        binArray.push(data[i].charCodeAt(0).toString(2)); 
    } 
    for (let j=0; j < binArray.length; j++) {
        var pad = padding_left(binArray[j], '0', 8);
        datEncode += pad + ' '; 
    }
    function padding_left(s, c, n) { if (! s || ! c || s.length >= n) {
        return s;
    }
    var max = (n - s.length)/c.length;
    for (var i = 0; i < max; i++) {
        s = c + s; } return s;
    }
    console.log(binArray);
  }

  const videoConstraints = {
    width: 1280,
    height: 720,
    facingMode: "user"
  };

  const WebcamCapture = () => (
    <Webcam
      audio={false}
      height={720}
      screenshotFormat="image/png"
      width={1280}
      videoConstraints={videoConstraints}
    >
      {({ getScreenshot }) => (
        <button
          onClick={() => {
            let imageSrc = getScreenshot();
            imageSrc = imageSrc.replace(/^data:image\/[a-z]+;base64,/, "");
            
            fetch("http://localhost:5000/send_qr",
            {
              'method':'POST',
              'headers' : {
                'Content-Type':'application/json',
                'Accepts':'application/json'
              },
              'body': JSON.stringify({
                "image":imageSrc
              })
            }
            ).then((response) => {
              console.log(response)
            })
            //console.log(imageSrc);
            //const blobData = atob(imageSrc);
           /*const req = new XMLHttpRequest();
            req.open("POST", "http://localhost:5000/send_qr", true);
            req.onload = (event) => {
              console.log(event);
            }
            req.send(blob);*/
          }}
        >
          Capture photo
        </button>
      )}
    </Webcam>
  );

  return (
    <>
     {WebcamCapture()}
    </>
  )
}

export default App
