# GUI Window Design
GUI windows shall contain the following:
- `PROGRAM_ICON`: This is a **REQUIRED** immutable icon to represent the program.
- `PROGRAM_NAME`: This is a **REQUIRED** immutable unique title of the program.
- `STATUS_WIDGETS`: This is an **OPTIONAL** list of widgets that go in the status bar at the bottom of the screen.
- `LONG_MENU`: This is an **OPTIONAL** list of widgets that make a menu for switching between different parts of the program (preferences, find & replace, etc.).  Once a tab is pressed, the sidebar goes back into hiding.
- `FILE_NAME`: This is inside of menubar if it exists.  Pressing opens file chooser (except on MacOS due to thin menu bar, use: *File -> Open* instead).
- `MENU_BAR`: This is an **OPTIONAL** list of icons at the top of the window.

Fullscreen always acts as DiveOS fullscreen.  Enter fullscreen by pressing F11, or on MacOS the fullscreen icon in the menu bar.

## DiveOS
Notes:
- DiveOS is always fullscreen, so there's only 1 render mode (F11 not needed)
- If there are no `STATUS_WIDGETS` the status bar becomes a watermark overlay showing `PROGRAM_NAME` and `Battery TimeDate`.  You can hide it (in DiveOS app settings) for an app or all apps if you don't like it.

```
+--------------|--------------------------------------------------+
| PROGRAM_ICON |                                 Battery TimeDate |
| PROGRAM_NAME |-----------|--------------------------------------|
| LONG_MENU    | FILE_NAME   MENU_BAR                 AppSwitcher |
|              |--------------------------------------------------|
|              | WIDGET_LIST                                      |
|              |--------------------------------------------------|
|              | STATUS_WIDGETS                                   | # Status Bar
+--------------|--------------------------------------------------+
```

## Linux
```
+------------|--------------|----------------|---------------------+
| Activities   PROGRAM_NAME   Month Day Time   Wifi Volume Battery |
|------------------------------------------------------------------|
|                                                                X |
|------------------------------------------------------------------|
| STATUS_WIDGETS                                                   | # Status Bar
+------------------------------------------------------------------+
```

## MacOS
```
+------|--------------|--------------------------+
| Menu   PROGRAM_NAME   Volume Wifi Battery Time |
|---------------------------|--------------------|
| Close Minimize Fullscreen   FILE_NAME          |
|------------------------------------------------|
| 
|------------------------------------------------|
| STATUS_WIDGETS                                 | # Status Bar
+------------------------------------------------+
```

## Windows
```
+---------------------------------------------------------------+
| 
|--------------|------------------------------------------------|
| PROGRAM_NAME   STATUS_WIDGETS                                 | # Status Bar
|-----------------|---------------------------------------------|
| Windows Cortana   App_Icons Volume Battery Time Notifications |
+---------------------------------------------------------------+
```

## Android
```
+---------|-----------|----------------------------+
| SIDEBAR | FILE_NAME   MENU_BAR                   |
|--------------------------------------------------|
|
|--------------|----------------|------------------|
| PROGRAM_NAME   STATUS_WIDGETS   Battery TimeDate | # Status Bar
+--------------------------------------------------+
```

## iOS
```
+---------|-----------|----------------------------+
| SIDEBAR | FILE_NAME   MENU_BAR                   |
|--------------------------------------------------|
|
|--------------|----------------|------------------|
| PROGRAM_NAME   STATUS_WIDGETS   Battery TimeDate | # Status Bar
+--------------------------------------------------+
```
