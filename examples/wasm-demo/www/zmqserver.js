var app = require('express')();
var server = require('http').Server(app);
var io = require('socket.io')(server);

io.on('test', function (socket) {
  socket.on('chat message', function (msg) {
    io.emit('test', msg);
  });
  io.emit('test', 'Neuer Client verbunden')
});

const zmq = require('zeromq')
const sock = new zmq.Subscriber


// sock.connect('tcp://perma-1.iota.partners:5556')
sock.connect('tcp://zmq.devnet.iota.org:5556')
sock.subscribe('tx')

run()
const maxmessages = 30
const messages = []
let hashes = []
async function run(){
  for await (const [topic, msg] of sock) {
    // console.log("received a message related to:", topic.toString())
    const data = topic.toString().split(' ') // Split to get topic & data
  switch (
  data[0] // Use index 0 to match topic
  ) {
    case 'tx':
      let txhash = data[1]
      let trunk = data[9]
      let branch = data[10]
      io.emit('tx', {[txhash]:trunk+branch})
  }
  }
}

io.on('connection', socket => {
  for (var i = 0, len = messages.length; i < len; i++) {
    io.emit('tx', messages[i])
  }
  console.log(socket.request.connection.remoteAddress + ' connected');
  socket.on('disconnect', () => { console.log(socket.request.connection.remoteAddress + ' disconnected'); });
});

io.on('test', data => {
  console.log(data);
})

server.listen(8081, () => {
  console.log('listening on *:8081');
}) 
