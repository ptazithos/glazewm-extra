import { invoke } from "@tauri-apps/api";
import type { Optional } from "./utils";
import { WindowRule, parseCommand } from "./rules";

export type WindowInfo = {
	hwnd: number;
	title: string;
	className: string;
	processName: string;
};

export const getWindowInfo = async (rawHandle: number): Promise<WindowInfo> => {
	const [_title, _className, _processName] = await Promise.all([
		invoke<string | null>("get_window_title", { rawHandle }),
		invoke<string | null>("get_window_class", { rawHandle }),
		invoke<string | null>("get_window_process_name", { rawHandle }),
	]);

	const title = _title ?? "";
	const className = _className ?? "";
	const processName = (_processName ?? "").split("\\").at(-1) ?? "";

	return {
		hwnd: rawHandle,
		title,
		className,
		processName,
	};
};

export type RawWindowRule = {
	command: string;
	match_process_name?: string;
	match_class_name?: string;
	match_title?: string;
};

export type RawAppConfig = Optional<{
	window_rules: Array<RawWindowRule>;
	focused_window_rules: Array<RawWindowRule>;
	unfocused_window_rules: Array<RawWindowRule>;
}>;

export type AppConfig = {
	windowRules: Array<WindowRule>;
	focusedWindowsRules: Array<WindowRule>;
	unfocusedWindowsRules: Array<WindowRule>;
};

export const getAppConfig = async (): Promise<AppConfig> => {
	const appConfig = (await invoke<RawAppConfig>("get_app_config")) ?? [];
	const _windowRules = appConfig.window_rules ?? [];
	const _focusedWindowRules = appConfig.focused_window_rules ?? [];
	const _unfocusedWindowRules = appConfig.unfocused_window_rules ?? [];

	const windowRules = _windowRules
		.map((rule) => {
			const command = rule?.command ? parseCommand(rule.command) : null;
			return command
				? new WindowRule(
						command,
						rule.match_process_name,
						rule.match_class_name,
						rule.match_title,
					)
				: null;
		})
		.filter((rule) => rule !== null) as WindowRule[];

	const focusedWindowsRules = _focusedWindowRules
		.map((rule) => {
			const command = rule?.command ? parseCommand(rule.command) : null;
			return command
				? new WindowRule(
						command,
						rule.match_process_name,
						rule.match_class_name,
						rule.match_title,
					)
				: null;
		})
		.filter((rule) => rule !== null) as WindowRule[];

	const unfocusedWindowsRules = _unfocusedWindowRules
		.map((rule) => {
			const command = rule?.command ? parseCommand(rule.command) : null;
			return command
				? new WindowRule(
						command,
						rule.match_process_name,
						rule.match_class_name,
						rule.match_title,
					)
				: null;
		})
		.filter((rule) => rule !== null) as WindowRule[];

	return {
		windowRules,
		focusedWindowsRules,
		unfocusedWindowsRules,
	};
};
