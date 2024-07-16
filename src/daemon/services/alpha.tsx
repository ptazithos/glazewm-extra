import { invoke } from "@tauri-apps/api";
import { subscribeFocusChanged } from "../../ipc/subscribe";
import { useEffect } from "react";
import type { AppConfig } from "../../native";
import { getWindows } from "../../ipc/command";
import { getWindowInfo } from "../../native";
import { info } from "tauri-plugin-log-api";

const AlphaService = (props: { config: AppConfig }) => {
	const rules = props.config.windowRules.filter(
		(rule) => rule.command.category === "translucent",
	);

	useEffect(() => {
		const setWindowsAlpha = async () => {
			const windows = await getWindows();
			for (const window of windows) {
				const hwnd = window?.handle;
				if (hwnd) {
					const windowInfo = await getWindowInfo(hwnd);
					for (const rule of rules) {
						rule.apply(windowInfo);
					}
				}
			}
		};

		const handle = subscribeFocusChanged(async (payload) => {
			const hwnd = payload?.data?.focusedContainer?.handle;
			if (hwnd) {
				await setWindowsAlpha();
				await invoke("set_window_alpha", { rawHandle: hwnd, alpha: 255 });
			}
		});

		return () => {
			handle.close();
		};
	}, [rules]);

	return <></>;
};

export default AlphaService;
