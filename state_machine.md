```text
+----------------------+
|  Boot / Idle         |
|  Sensor active       |
+----------+-----------+
           |
           | every 3 seconds
           v
+----------------------+
|  Sensing State       |
|  Wait for motion     |
+----------+-----------+
           |
           | motion detected
           v
+----------------------+
|  Motion Detected State|
|  Camera ON           |
|  Sensor OFF          |
+----------+-----------+
           |
           | after 10 seconds
           v
+----------------------+
|  Play sound.mp3      |
+----------+-----------+
           |
           | after 40 seconds
           v
+----------------------+
|  Sensing State       |
|  Camera OFF          |
|  Sensor resumes      |
+----------------------+
```
