import { invoke } from "@tauri-apps/api";
import { info } from "tauri-plugin-log-api";

import type { WindowInfo } from "./native";

export type RuleCommand<M, N> = {
	type: "set";
	category: M;
	value: N;
};

export type TraslucentCommand = RuleCommand<"translucent", number>;
export type TitleCommand = RuleCommand<"title", boolean>;

export class WindowRule {
	command: TraslucentCommand | TitleCommand;
	match_process_name?: string;
	match_class_name?: string;
	match_title?: string;

	constructor(
		command: TraslucentCommand | TitleCommand,
		match_process_name?: string,
		match_class_name?: string,
		match_title?: string,
	) {
		this.command = command;
		this.match_process_name = match_process_name;
		this.match_class_name = match_class_name;
		this.match_title = match_title;
	}

	apply(windowInfo: WindowInfo) {
		const isProcessNameMatched =
			this.match_process_name &&
			new RegExp(this.match_process_name).test(windowInfo.processName);
		const isClassNameMatched =
			this.match_class_name &&
			new RegExp(this.match_class_name).test(windowInfo.className);

		const isTitleMatched =
			this.match_title && new RegExp(this.match_title).test(windowInfo.title);

		const isMatched =
			isProcessNameMatched || isClassNameMatched || isTitleMatched;

		if (isMatched) {
			switch (this.command.category) {
				case "translucent":
					invoke("set_window_alpha", {
						rawHandle: windowInfo.hwnd,
						alpha: this.command.value,
					});
					break;
				case "title":
					invoke("set_window_titlebar", {
						rawHandle: windowInfo.hwnd,
						titlebar: this.command.value,
					});
					break;
				default:
					info(JSON.stringify(this.command));
			}
		}
	}
}
