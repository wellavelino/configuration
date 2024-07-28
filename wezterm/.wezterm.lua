-- Pull in the wezterm API
local wezterm = require("wezterm")
local mux = wezterm.mux
local config = {}

wezterm.on("gui-startup", function(cmd)
	local tab, pane, window = mux.spawn_window(cmd or {})
	window:gui_window():maximize()
end)

-- This will hold the configuration.
config = wezterm.config_builder()

-- This is where you actually apply your config choices

-- For example, changing the color scheme:
config.color_scheme = "terafox"

config.window_background_opacity = 1.0
config.font = wezterm.font("JetBrains Mono")
-- and finally, return the configuration to wezterm
return config
