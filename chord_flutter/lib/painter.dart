import 'dart:io';

import 'package:chord_flutter/dialog.dart';
import 'package:chord_flutter/network_utils.dart';
import 'package:flutter/material.dart';
import 'dart:math' as math;
import 'package:chord_flutter/node_painter.dart';
import 'package:chord_flutter/line_painter.dart';

class ChordRing extends StatefulWidget {
  final int initialNumNodes;

  const ChordRing({Key? key, required this.initialNumNodes}) : super(key: key);

  @override
  _ChordRingState createState() => _ChordRingState();
}

class _ChordRingState extends State<ChordRing> {
  int numNodes = 5;
  final nodeMap = {};
  List<int> nodes = [1];
  NetworkUtils networkUtils = NetworkUtils();

  Map<int, List<int>> nodeRelatives = {};

  List<int> nodeToShow = [];
  int? fingerTable;
  TextEditingController controller = TextEditingController();

  @override
  void initState() {
    super.initState();
    numNodes = widget.initialNumNodes;
    networkUtils.UDPSocketService();
    networkUtils.startListening('localhost', 3000);
    networkUtils.responseStream.listen((event) {
      for (int key in event.keys) {
        nodeRelatives[key] = event[key]!;
      }

      setState(() {});
    });
  }

  @override
  void dispose() {
    networkUtils.closeSocket();
    controller.dispose();
    super.dispose();
  }

  void _addNode() {
    showDialog(
        context: context,
        builder: (builder) => NodeAddDialog(
            networkUtils: networkUtils,
            onAddNode: (int nodeId) {
              nodes.add(nodeId);
              setState(() {});
            },
            nodes: nodes));
  }

  void _onNodeTap(int nodeId) {
    // Handle node tap event (e.g., print node ID)
    if (nodeToShow.contains(nodeId)) {
      nodeToShow.remove(nodeId);
    } else {
      nodeToShow.add(nodeId);
    }
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    //List<Offset> nodePositions = [];
    double screenWidth = MediaQuery.of(context).size.width;
    double screenHeight = MediaQuery.of(context).size.height;
    final center = Offset(screenWidth / 2, screenHeight / 2);

    // Calculate angular position and (x, y) coordinates for each node
    final double anglePerNode = 2 * math.pi / nodes.length;
    nodes.sort();
    for (int i = 0; i < nodes.length; i++) {
      final double angle = i * anglePerNode;
      final double x = center.dx + 200 * math.sin(angle);
      final double y = center.dy - 200 * math.cos(angle);
      nodeMap[nodes[i]] = Offset(x, y);
    }
    return Scaffold(
        backgroundColor: Colors.black,
        body: Stack(
          children: [
            Positioned(
                top: 5,
                left: screenWidth / 2 - 55,
                child: ElevatedButton(
                    onPressed: _addNode, child: const Text('Add Node'))),
            CustomPaint(
              painter: ArrowPainter(
                nodePositions: nodeMap,
                nodeRelations: nodeRelatives,
                nodeToShow: nodeToShow,
              ),
            ),
            Positioned(
                left: 10,
                top: 100,
                child: Column(children: [
                  Row(children: [
                    SizedBox(
                      width: 100,
                      height: 40,
                      child: TextField(
                        style: const TextStyle(color: Colors.white),
                        controller: controller,
                        decoration: const InputDecoration(
                          hintText: "Node ID of finger table",
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.all(Radius.circular(10)),
                          ),
                        ),
                      ),
                    ),
                    const SizedBox(
                      width: 5,
                    ),
                    ElevatedButton(
                        onPressed: () {
                          setState(() {
                            fingerTable = int.parse(controller.text.trim());
                          });
                        },
                        child: const Text("go"))
                  ]),
                  if (fingerTable != null &&
                      nodeRelatives[fingerTable!] != null)
                    fingertable(nodeRelatives[fingerTable!]!, fingerTable!)
                ])),
            chordRingPainter(
                onNodeTap: _onNodeTap,
                context: context,
                nodePositions: nodeMap,
                radius: 200.0),
          ],
        ));
  }
}

Widget fingertable(List<int> nodeTable, int nodeid) {
  return Column(
    children: [
      const SizedBox(
        height: 10,
      ),
      Text(
        "finger table of $nodeid",
        style:
            const TextStyle(color: Colors.white, fontWeight: FontWeight.w500),
      ),
      const SizedBox(
        height: 10,
      ),
      for (int i = 0; i < nodeTable.length; i++)
        Column(
          children: [
            Row(children: [
              Text(
                "$i ---->",
                style: const TextStyle(
                    color: Colors.white, fontWeight: FontWeight.w300),
              ),
              const SizedBox(
                width: 5,
              ),
              Text((nodeTable[i] != -1) ? "${nodeTable[i]}" : "None",
                  style: const TextStyle(
                      color: Colors.white, fontWeight: FontWeight.w300)),
            ]),
            const SizedBox(
              height: 5,
            ),
          ],
        )
    ],
  );
}
