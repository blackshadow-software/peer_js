const peer = new Peer();

let localStream;
let currentCall;

peer.on("open", (id) => {
  document.getElementById("my-id").textContent = id;
  // Get webcam
  navigator.mediaDevices
    .getUserMedia({ video: true, audio: true })
    .then((stream) => {
      localStream = stream;
      document.getElementById("my-video").srcObject = stream;
    })
    .catch((err) => console.error("Failed to get local stream", err));
});

function startCall() {
  const peerId = document.getElementById("peer-id").value;
  const call = peer.call(peerId, localStream);
  currentCall = call;

  call.on("stream", (remoteStream) => {
    document.getElementById("remote-video").srcObject = remoteStream;
  });

  call.on("close", () => alert("Call ended"));
}

function startScreenShareCall() {
  const peerId = document.getElementById("peer-id").value;

  navigator.mediaDevices
    .getDisplayMedia({ video: true, audio: true })
    .then((screenStream) => {
      const call = peer.call(peerId, screenStream);
      currentCall = call;

      document.getElementById("my-video").srcObject = screenStream;

      call.on("stream", (remoteStream) => {
        document.getElementById("remote-video").srcObject = remoteStream;
      });

      call.on("close", () => alert("Call ended"));

      // Optional: Stop screen sharing when the stream ends
      screenStream.getVideoTracks()[0].addEventListener("ended", () => {
        alert("Screen sharing stopped.");
        call.close();
      });
    })
    .catch((err) => {
      console.error("Failed to get display media:", err);
      alert("Screen sharing was blocked or failed.");
    });
}

peer.on("call", (call) => {
  call.answer(localStream); // Answer the call with our stream
  currentCall = call;

  call.on("stream", (remoteStream) => {
    document.getElementById("remote-video").srcObject = remoteStream;
  });

  call.on("close", () => alert("Call ended"));
});
