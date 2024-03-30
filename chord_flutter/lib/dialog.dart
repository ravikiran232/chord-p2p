import 'dart:io';

import 'package:chord_flutter/network_utils.dart';
import 'package:flutter/material.dart';

class NodeAddDialog extends StatefulWidget {
  final NetworkUtils networkUtils;
  final Function(int) onAddNode;
  final List<int> nodes;

  const NodeAddDialog({
    super.key,
    required this.networkUtils,
    required this.onAddNode,
    required this.nodes,
  });

  @override
  State<NodeAddDialog> createState() => _NodeAddDialogState();
}

class _NodeAddDialogState extends State<NodeAddDialog> {
  late TextEditingController _controller;
  bool _showError = false;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text("Enter the node ID to add"),
      content: TextField(
        controller: _controller,
        decoration: InputDecoration(
          hintText: "Node ID",
          border: const OutlineInputBorder(
            borderRadius: BorderRadius.all(Radius.circular(10)),
          ),
          errorText: _showError
              ? "Node already exists"
              : null,
        ),
      ),
      actions: [
        TextButton(
          onPressed: () {
            Navigator.of(context).pop();
          },
          child: const Text("Cancel"),
        ),
        TextButton(
          onPressed: () {
            final int nodeId = int.tryParse(_controller.text.trim()) ?? -1;
            if (nodeId >= 256) {
              setState(() {
                _showError = true;
              });
            }
            else if (!widget.nodes.contains(nodeId)) {
              widget.networkUtils.sendRequest(
                  _controller.text.trim(), InternetAddress("127.0.0.1"), 8080);
              widget.onAddNode(nodeId);
              Navigator.of(context).pop();
            } else {
              // Show error if the node already exists
              setState(() {
                _showError = true;
              });
            }
          },
          child: const Text("Add Node"),
        ),
      ],
    );
  }
}
