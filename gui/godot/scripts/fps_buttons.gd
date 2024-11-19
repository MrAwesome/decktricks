extends HBoxContainer

# TODO: store settings in json file somewhere
# TODO: fix selection with controller (focus stealing happening from Main.gd)

const DEFAULT_FPS_INDEX = 1
const selected_color = Color(0.675, 0.412, 0.0)

const fps_values = ["15", "30", "45", "60", "90", "VSync"]

func set_fps(val: String, butt: Button):
	if val.is_valid_int():
		Engine.set_max_fps(int(val))
	else:
		Engine.set_max_fps(0)
	
	for button in get_children():
		button.remove_theme_color_override("font_color")
		butt.add_theme_color_override("font_color", selected_color)
		butt.add_theme_color_override("font_focus_color", selected_color)
		butt.add_theme_color_override("font_hover_color", selected_color)

func make_fps_button(val: String) -> Button:
	var butt = Button.new()
	butt.text = val
	butt.add_theme_color_override("font_hover_pressed_color", selected_color)
	butt.add_theme_color_override("font_pressed_color", selected_color)

	butt.pressed.connect(set_fps.bind(val, butt))
	return butt

func _ready() -> void:
	var i = 0
	for val in fps_values:
		var butt = make_fps_button(val)
		if i == DEFAULT_FPS_INDEX:
			set_fps(val, butt)
		add_child(butt)
		i += 1
