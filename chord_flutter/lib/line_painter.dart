import 'dart:math' as math;
import 'package:flutter/material.dart';

class ArrowPainter extends CustomPainter {
  final Map nodePositions;
  final Map<int,List<int>> nodeRelations;
  final double arrowSize;
  final double arrowAngle;
  final List<int> nodeToShow;

  ArrowPainter({
    required this.nodePositions,
    required this.nodeRelations,
    this.arrowSize = 15,
    required this.nodeToShow,
    this.arrowAngle = 25 * math.pi / 180,
  });

  @override
  void paint(Canvas canvas, Size size) {
    // Draw the line
    final paint = Paint()
      ..color = Colors.yellow
      ..strokeWidth = 3
      ..style = PaintingStyle.stroke;
    for (int i in nodeToShow) {
      if (nodeRelations[i] == null) {
        continue;
      }
      Offset p1 = nodePositions[i];
      for (int j = 0; j < nodeRelations[i]!.length; j++) {
        if (nodeRelations[i]![j] == -1) {
          continue;
        }
        Offset p2 = nodePositions[nodeRelations[i]![j]];
        // print("$p1,$p2");
        canvas.drawLine(p1, p2, paint);

        // Calculate the angle between the points
        final dX = p2.dx - p1.dx;
        final dY = p2.dy - p1.dy;
        final angle = math.atan2(dY, dX);

        // Draw the arrowhead
        final path = Path();
        path.moveTo(p2.dx - arrowSize * math.cos(angle - arrowAngle),
            p2.dy - arrowSize * math.sin(angle - arrowAngle));
        path.lineTo(p2.dx, p2.dy);
        path.lineTo(p2.dx - arrowSize * math.cos(angle + arrowAngle),
            p2.dy - arrowSize * math.sin(angle + arrowAngle));
        path.close();
        canvas.drawPath(path, paint);
      }
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }
}