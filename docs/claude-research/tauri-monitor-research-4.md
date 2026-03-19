# System Resource Monitoring Applications Research

## Desktop System Monitors with CPU, Network, and Disk Monitoring

### Cross-Platform Applications

1. **htop** - Enhanced version of top with better visualization
   - Features: CPU, memory, process monitoring (no native network monitoring)
   - Platform: Linux, macOS (Windows via WSL, not native)
   - Type: Terminal-based

2. **btop++** - Modern C++ version of htop
   - Features: CPU, memory, network, disk I/O monitoring
   - Platform: Linux, macOS, Windows
   - Type: Terminal-based with rich UI

3. **Glances** - Cross-platform system monitoring tool
   - Features: CPU, memory, network, disk, sensors
   - Platform: Linux, macOS, Windows
   - Type: Terminal and web interface

4. **NetData** - Real-time performance monitoring
   - Features: CPU, memory, network, disk, comprehensive metrics
   - Platform: Linux, macOS, Windows
   - Type: Web-based dashboard

5. **Stacer** - Linux system optimizer and monitoring tool
   - Features: CPU, memory, disk usage, network monitoring
   - Platform: Linux only
   - Type: GUI application

### macOS Specific

6. **Activity Monitor** - Built-in macOS system monitor
   - Features: CPU, memory, energy, disk, network
   - Platform: macOS only
   - Type: Native GUI

7. **iStat Menus** - Advanced system monitor for macOS
   - Features: CPU, memory, disk, network, sensors, battery
   - Platform: macOS only
   - Type: Menu bar application

8. **MenuMeters** - Open source system monitoring for macOS
   - Features: CPU, memory, disk, network in menu bar
   - Platform: macOS only
   - Type: Menu bar application

9. **Stats** - Free macOS system monitor
   - Features: CPU, memory, disk, network, sensors
   - Platform: macOS only
   - Type: Menu bar application

### Windows Specific

10. **Task Manager** - Built-in Windows system monitor
    - Features: CPU, memory, disk, network, processes
    - Platform: Windows only
    - Type: Native GUI

11. **Process Explorer** - Advanced process and system monitor
    - Features: CPU, memory, handles, DLLs
    - Platform: Windows only
    - Type: GUI application

12. **Resource Monitor** - Built-in Windows resource monitoring
    - Features: CPU, memory, disk, network detailed view
    - Platform: Windows only
    - Type: Native GUI

13. **HWiNFO** - Hardware information and monitoring
    - Features: CPU, memory, disk, network, sensors
    - Platform: Windows only
    - Type: GUI application

### Linux Specific

14. **System Monitor (GNOME)** - Default GNOME system monitor
    - Features: CPU, memory, network, processes
    - Platform: Linux (GNOME)
    - Type: GUI application

15. **KSysGuard** - KDE system monitor
    - Features: CPU, memory, network, disk, processes
    - Platform: Linux (KDE)
    - Type: GUI application

16. **Conky** - Lightweight system monitor
    - Features: CPU, memory, disk, network, customizable
    - Platform: Linux, FreeBSD
    - Type: Desktop widget

### Modern Cross-Platform Solutions

17. **Electron-based Monitors**
    - Various applications built with Electron
    - Features: CPU, memory, disk, network
    - Platform: Cross-platform
    - Type: Desktop applications

18. **Tauri-based Monitors** - Rust + web frontend
    - Emerging applications using Tauri framework
    - Features: System resource monitoring
    - Platform: Cross-platform
    - Type: Native performance with web UI

19. **Flutter Desktop Monitors**
    - Applications built with Flutter for desktop
    - Features: System monitoring capabilities
    - Platform: Cross-platform
    - Type: Native desktop applications

### Terminal-Based Advanced Tools

20. **nmon** - Performance monitoring tool
    - Features: CPU, memory, network, disk I/O
    - Platform: Linux, AIX
    - Type: Terminal-based

21. **iotop** - I/O monitoring tool
    - Features: Disk I/O monitoring by process
    - Platform: Linux
    - Type: Terminal-based

22. **nethogs** - Network monitoring by process
    - Features: Network bandwidth usage per process
    - Platform: Linux
    - Type: Terminal-based

23. **iftop** - Network interface monitoring
    - Features: Real-time network bandwidth usage
    - Platform: Linux, macOS
    - Type: Terminal-based

### Web-Based Solutions

24. **Grafana + Prometheus** - Monitoring stack
    - Features: Comprehensive system metrics
    - Platform: Cross-platform
    - Type: Web-based dashboard

25. **Zabbix** - Enterprise monitoring solution
    - Features: CPU, memory, network, disk monitoring
    - Platform: Cross-platform
    - Type: Web-based interface

## Summary

Total projects found: **25 monitoring applications**

Categories:
- Cross-platform: 7 applications (Stacer reclassified to Linux)
- macOS specific: 4 applications
- Windows specific: 4 applications
- Linux specific: 4 applications (including Stacer)
- Modern frameworks: 3 applications
- Terminal-based: 4 applications
- Web-based: 2 applications

All listed applications provide comprehensive monitoring of CPU, network, and disk resources with varying levels of detail and user interface approaches.