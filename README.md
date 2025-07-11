# app-runner-and-waiter
A CLI tool for running an executable, wait for it to open its port, then open the specifier port in a browser.

This is mainly for web-ui's such as SillyTavern/Automatic1111 etc.

For example, adding it to your KDE application menu.

## Usage
Simply download the released executable, place it somewhere.
Create the KDE/whatever application menu with the arguments setup.

## arguments
`-a/--app`: The executable to run, for example: `-a /home/username/Applications/SillyTavern/start.sh`

`-p/--port`: The port to wait and then open, for example: `-p 8000`

`-o/--opener`: How it should open the URL, for example: `-o xdg-open` (default)

`-t/--timeout`: How many seconds it should wait for the application to open, for example: `-t 30` (default)

`-d/--directory`: The directory to run the application in, by default it is the parent of `--app`, for example: `/home/username/Applications/SillyTavern`

`-n/--no-open`: Don't open a URL once opened. Default false. Used for using the app as a launcher ig.

`-b/--base-url`: The base URL to open (hostname), default is `127.0.0.1`

`-h/--help`: The help

All in all, for example, if you have Automatic1111 in `~/Applications`, the following will work:

`app-runner-and-waiter -a /home/username/Applications/Automatic1111/webui.sh -p 7860`

Proper examples you'd add in KDE application menu:

`konsole -e bash -c 'app-runner-and-waiter -a /home/username/Applications/SillyTavern/start.sh -p 8000'`

`konsole -e bash -c 'app-runner-and-waiter --app /home/username/Applications/Automatic1111/webui.sh --no-open --port 7860 -- --api'`

And that should work.
