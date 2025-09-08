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

var initializing = true
var INFO_WINDOW = preload("res://scenes/info_window.tscn")

signal restart_steam_hint

func send_steam_restart_hint():
	emit_signal("restart_steam_hint")

func initialize_action_button(
	action_button: ActionButton,
):
	action_button.button_original_color = action_button.modulate
	action_button.focus_entered.connect(focus_button.bind(action_button))
	action_button.focus_exited.connect(rejigger_focus_on_visibility_loss.bind(action_button).call_deferred)

func rejigger_focus_on_visibility_loss(action_button: ActionButton):
	if !action_button.is_visible_in_tree():
		var sibs = action_button.get_parent().get_children()
		for sib in sibs:
			if sib.is_visible_in_tree():
				sib.grab_focus.call_deferred()
				break

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
	is_completed: bool,
) -> void:
	action_button.set_name(identifier)
	action_button.set_text(display_text)
	action_button.set_visible(is_available)

	if is_completed:
		action_button.modulate = Color.DARK_GRAY

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
	dd.log(4, "UI refresh update sent.")

func _on_log_refresh_timer_timeout() -> void:
	%LogContainer.populate_logs()

func _on_context_was_updated() -> void:
	dd.update_all_buttons(get_tree())

func _on_show_info_window(info: Dictionary) -> void:
	popup_info_window(info)

func _on_should_restart_decktricks_gui() -> void:
	# NOTE: should maybe only exit if this file is actually removed:
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

	# Handle L1/R1 moving among the main tabs and update button
	# NOTE: could focus the first element of the first subtab here if desired
	if event.is_action_pressed("ui_prev_main_tab"):
		var did_change_tab = %MainTabs.select_previous_available()
		if did_change_tab:
			%MainTabs.get_tab_bar().grab_focus()
		elif $UpdateButton.visible:
			$UpdateButton.grab_focus()
	if event.is_action_pressed("ui_next_main_tab"):
		if not $UpdateButton.has_focus():
			%MainTabs.select_next_available()
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
	dd.added_to_steam.connect(send_steam_restart_hint.call_deferred)

	var should_test = OS.get_environment("DECKTRICKS_GUI_TEST_COMMAND_ONLY")
	var should_exit = OS.get_environment("DECKTRICKS_GUI_EXIT_IMMEDIATELY")

	if should_test:
		var test_cmd_args: Array[String]
		test_cmd_args.assign(should_test.split("|DELIM|"))
		dd.sync_run_with_decktricks(test_cmd_args)

	%LogContainer.populate_logs()
	dd.populate_categories(%Categories)

	%Categories.select_next_available()

	var main_tab_bar: TabBar = %MainTabs.get_tab_bar()
	main_tab_bar.set_focus_neighbor(SIDE_LEFT, $UpdateButton.get_path())
	$UpdateButton.set_focus_neighbor(SIDE_RIGHT, main_tab_bar.get_path())
	$UpdateButton.set_focus_neighbor(SIDE_LEFT, '')

	var first_button = get_tree().get_nodes_in_group("first_button").pop_front()
	if first_button:
		first_button.grab_focus.call_deferred()

	dd.sync_run_with_decktricks(["version", "--verbose"])
	dd.log(2, "Decktricks GUI initialization complete!")

	# NOTE: This line should be the last non-testing output before exit, otherwise integration tests will fail:
	print("Decktricks GUI initialization complete!")

	# If requested by tests, emulate user inputs via action presses
	var test_inputs = OS.get_environment("DECKTRICKS_GUI_TEST_INPUTS")
	if test_inputs:
		var actions: Array[String]
		actions.assign(test_inputs.split("|DELIM|"))
		var exit_after_inputs = OS.get_environment("DECKTRICKS_GUI_EXIT_AFTER_INPUTS")
		var post_delay_ms = OS.get_environment("DECKTRICKS_GUI_INPUTS_POST_DELAY_MS")
		call_deferred("_perform_test_actions", actions, exit_after_inputs, post_delay_ms)

	if should_exit:
		get_tree().quit()

func _perform_test_actions(actions: Array[String], exit_after_inputs: String, post_delay_ms: String) -> void:
	for action_name in actions:
		var ev_press := InputEventAction.new()
		ev_press.action = action_name
		ev_press.pressed = true
		Input.parse_input_event(ev_press)
		await get_tree().create_timer(0.05).timeout

		var ev_release := InputEventAction.new()
		ev_release.action = action_name
		ev_release.pressed = false
		Input.parse_input_event(ev_release)
		await get_tree().create_timer(0.05).timeout

	if exit_after_inputs:
		var delay_ms := 300
		if post_delay_ms:
			delay_ms = int(post_delay_ms)
		await get_tree().create_timer(float(delay_ms) / 1000.0).timeout
		get_tree().quit()
