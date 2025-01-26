extends Control

# TODO: some kind of error display system (emit signal and handle it by
# 		displaying an AcceptDialog with the text and a report link/QR,
# 		and have a timeout for how many errors can be shown at a time
# 		(or how quickly)), and have an exit program option from errors
# TODO: fix going up from info sometimes going to tabs
# 		instead of previous trick's buttons
# TODO: handle 720p since that's a common resolution on TVs?
# TODO: use this to set the STEAM_ID as needed for gamescope? Window.set_flag
# TODO: improve visual "you did it" cues for "Add to Steam" (either have a check if it's already
#		added to steam, or just change the button to "yeah yay added" and flash green?

const DEFAULT_MAX_FPS = 30
const UPDATE_FILE = "/tmp/decktricks_did_update"
var dd = DecktricksDispatcher

var lol = Color.from_rgba8(0,0,0,0)

var initializing = true
var INFO_WINDOW = preload("res://scenes/info_window.tscn")

signal restart_steam_hint

func initialize_action_button(
	action_button: ActionButton,
):
	action_button.button_original_color = action_button.modulate
	action_button.focus_entered.connect(focus_button.bind(action_button))

# On button focus, make sure that at least one row above can be focused (to fix scrolling up)
func focus_button(button: Button):
	var row = button.find_parent("RowOuterMargin").get_parent()
	var idx = row.get_index()
	var desired_idx = max(0, idx-1)
	var desired_row = row.get_parent().get_child(desired_idx)

	var scroller = button.find_parent("TricksScroller")
	scroller.ensure_control_visible(desired_row)

	# Store the focused button to be re-focused on refresh
	#focused_trick_and_action = [trick_id, action]

func update_action_button(
	action_button: ActionButton,
	identifier: String,
	display_text: String,
	is_available: bool,
	is_ongoing: bool,
) -> void:
	action_button.set_name(identifier)
	action_button.set_text(display_text)
	action_button.set_visible(is_available)

	# TODO: make not clickable while running
	if is_ongoing:
		if not action_button.button_known_ongoing_state:
			action_button.button_known_ongoing_state = true
			action_button.disabled = true

			var tween = create_tween()
			tween.set_loops()
			tween.tween_interval(0.1)
			var trans = Tween.TRANS_QUAD
			tween.tween_property(action_button, "modulate", Color.GREEN, 2) \
				.set_ease(Tween.EASE_IN_OUT).set_trans(trans)
			tween.tween_property(action_button, "modulate", Color.FOREST_GREEN, 2) \
				.set_ease(Tween.EASE_IN_OUT).set_trans(trans)
			tween.bind_node(action_button)

			action_button.button_tween = tween

	if not is_ongoing:
		if action_button.button_known_ongoing_state:
			action_button.button_known_ongoing_state = false
			action_button.disabled = false
			action_button.set_modulate(action_button.button_original_color)

			action_button.button_tween.kill()

func popup_info_window(info: Dictionary):
	var root = get_tree().root
	var dialog: AcceptDialog = INFO_WINDOW.instantiate()
	dialog.theme = theme
	dialog.get_ok_button().set_text("OK")

	dialog.set_title(info["title"])
	dialog.set_text(info["text"])

	root.add_child(dialog)
	dialog.popup_centered_ratio(0.8)

func _on_ui_refresh_timer_timeout() -> void:
	dd.async_refresh_system_context()

func _on_log_refresh_timer_timeout() -> void:
	%LogContainer.populate_logs()

func _on_context_was_updated() -> void:
	dd.update_all_buttons(get_tree())

func _on_show_info_window(info: Dictionary) -> void:
	popup_info_window(info)

func _on_should_restart_decktricks_gui() -> void:
	DirAccess.remove_absolute(UPDATE_FILE)
	# Exit with a special exit code that ../../build_assets/decktricks-gui.sh
	# will use to know whether to restart this program
	get_tree().quit(100)

func _on_update_check_timer_timeout() -> void:
	if FileAccess.file_exists(UPDATE_FILE):
		$UpdateButton.set_visible(true)

func _input(event: InputEvent) -> void:
	# If this window loses focus, do not accept any input (otherwise,
	# we would process gamepad input while child programs are in focus
	# which is a major problem in gamescope)
	if not DisplayServer.window_is_focused(0):
		accept_event()
		return

	if event.is_action_pressed("ui_exit_decktricks"):
		get_tree().quit()

	# NOTE: could focus the first element of the first subtab here if desired
	if event.is_action_pressed("ui_next_main_tab"):
		%MainTabs.select_next_available()
		%MainTabs.get_tab_bar().grab_focus()
	if event.is_action_pressed("ui_prev_main_tab"):
		%MainTabs.select_previous_available()
		%MainTabs.get_tab_bar().grab_focus()

func _init():
	dd.get_time_passed_ms("init")
	dd.run_startup_logic()
	Engine.set_max_fps(DEFAULT_MAX_FPS)
	dd.get_time_passed_ms("init_finished")

func _ready():
	dd.get_time_passed_ms("entered_ready")

	# Hook up signals, most of which are sent from the Rust side:
	dd.show_info_window.connect(_on_show_info_window)
	dd.context_was_updated.connect(_on_context_was_updated)
	dd.update_action_button.connect(update_action_button.call_deferred)
	dd.initialize_action_button.connect(initialize_action_button.call_deferred)
	dd.added_to_steam.connect(func (): emit_signal("restart_steam_hint"))

	var should_test = OS.get_environment("DECKTRICKS_GUI_TEST_COMMAND_ONLY")
	var should_exit = OS.get_environment("DECKTRICKS_GUI_EXIT_IMMEDIATELY")

	if should_test:
		var test_cmd_args: Array[String]
		test_cmd_args.assign(should_test.split("|DELIM|"))
		dd.sync_run_with_decktricks(test_cmd_args)

	%LogContainer.populate_logs()
	dd.populate_categories(%Categories)

	var first_button = get_tree().get_nodes_in_group("first_button").pop_front()
	if first_button:
		print("Grabbing focus...")
		print(first_button.text)
		first_button.grab_focus.call_deferred()

	var version_info = dd.sync_run_with_decktricks(["version", "--verbose"])
	dd.log(2, "Version info:\n" + version_info)

	dd.log(2, "Decktricks GUI initialization complete!")
	# This line should be last, otherwise integration tests will fail:
	print("Decktricks GUI initialization complete!")

	if should_exit:
		get_tree().quit()
		
		
		
	
