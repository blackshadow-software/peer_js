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

peer.on("call", (call) => {
  call.answer(localStream); // Answer the call with our stream
  currentCall = call;

  call.on("stream", (remoteStream) => {
    document.getElementById("remote-video").srcObject = remoteStream;
  });

  call.on("close", () => alert("Call ended"));
});
