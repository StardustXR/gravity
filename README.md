# Gravity
Utility to launch apps and stardust clients spatially
> [!IMPORTANT]  
> Requires the [Stardust XR Server](https://github.com/StardustXR/server) to be running. For launching 2D applications, [Flatland](https://github.com/StardustXR/flatland) also needs to be running.  

If you installed the Stardust XR server via:  
```note
sudo dnf group install stardust-xr
```
Or if you installed via the [installation script](https://github.com/cyberneticmelon/usefulscripts/blob/main/stardustxr_setup.sh), Gravity comes pre-installed

## How to Use 
An example of using Gravity would be running the command:
```bash
gravity 0 0.5 0 -r 120 <application>
```

Will spawn a window of the application at X Y Z and -r YAW. By default, if YAW is not set the window will be facing your current location. 

## Manual Installation
Clone the repository and after the server is running:
```bash
cargo run X Y Z -r YAW <application>
```
