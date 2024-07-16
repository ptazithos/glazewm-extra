import { useEffect } from "react";
import { getWindowInfo, type AppConfig } from "../../native";
import { invoke } from "@tauri-apps/api";
import { getWindows } from "../../ipc/command";
import { subscribeWindowManaged } from "../../ipc/subscribe";

const TitleService = (props: { config: AppConfig }) => {
	const rules = props.config.windowRules;

	useEffect(() => {
		const setWindowsTitleBar = async () => {
			const titleRules = rules.filter(
				(rule) => rule.command.category === "title",
			);

			const windows = await getWindows();
			for (const window of windows) {
				if (!window?.handle) continue;
				const info = await getWindowInfo(window.handle);
				for (const rule of titleRules) {
					const isClassNameMatched =
						rule.match_class_name &&
						new RegExp(rule.match_class_name).test(info.className);
					const isProcessNameMatched =
						rule.match_process_name &&
						new RegExp(rule.match_process_name).test(info.processName);
					const isTitleMatched =
						rule.match_title && new RegExp(rule.match_title).test(info.title);

					const isRuleMatched =
						isClassNameMatched || isProcessNameMatched || isTitleMatched;

					if (isRuleMatched) {
						invoke("set_window_titlebar", {
							rawHandle: window?.handle,
							titlebar: rule.command.value,
						});
					}
				}
			}
		};

		setWindowsTitleBar();

		const handle = subscribeWindowManaged(async (payload) => {
			setTimeout(() => {
				const hwnd = payload?.data?.managedWindow?.handle;
				hwnd &&
					invoke("set_window_titlebar", { rawHandle: hwnd, titlebar: false });
			}, 200);
		});

		return () => {
			handle.close();
		};
	}, [rules]);

	return <></>;
};

export default TitleService;
