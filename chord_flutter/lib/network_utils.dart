import "dart:io";
import 'dart:convert';
import "dart:async";

class NetworkUtils {
  late RawDatagramSocket _socket;
  late StreamController<Map<int, List<int>>> _responseStreamController;
  Stream<Map<int, List<int>>> get responseStream =>
      _responseStreamController.stream;

  UDPSocketService() {
    _responseStreamController =
        StreamController<Map<int, List<int>>>.broadcast();
  }

  Future<void> startListening(String host, int port) async {
    try {
      _socket = await RawDatagramSocket.bind(host, port);
      //_socket.send("hello".codeUnits, InternetAddress("127.0.0.1"), 8080);
      _socket.listen(_handleResponse);
    } catch (e) {
      print('Error binding to socket: $e');
    }
  }

  void _handleResponse(RawSocketEvent event) async{
    Datagram? datagram = _socket.receive();
    if (datagram != null) {
      String response = String.fromCharCodes(datagram.data);
      Map<int, List<int>> nodeRelation = parseResponse(response);
      //await Future.delayed(const Duration(milliseconds: 1000));
      _responseStreamController.add(nodeRelation);
    }
  }

  void sendRequest(String request, InternetAddress address, int port) {
    _socket.send(utf8.encode(request), address, port);
  }

  Map<int, List<int>> parseResponse(String response) {
    Map<int, List<int>> nodeRelations = {};
    List<dynamic> data = jsonDecode(response);
    List<int> relations = [];
    int key =data[0];
    for (int i = 0; i < data.length - 1; i++) {
      relations.add(data[i + 1]);
    }
    nodeRelations[key] = relations;
    return nodeRelations;
  }

  void closeSocket() {
    _socket.close();
    _responseStreamController.close();
  }
}
