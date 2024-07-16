import { useEffect } from "react";
import { getWindowInfo, type AppConfig } from "../../native";
import { invoke } from "@tauri-apps/api";
import { getWindows } from "../../ipc/command";
import { subscribeWindowManaged } from "../../ipc/subscribe";

const TitleService = (props: { config: AppConfig }) => {
	const rules = props.config.windowRules.filter(
		(rule) => rule.command.category === "title",
	);

	useEffect(() => {
		const setWindowsTitleBar = async () => {
			const windows = await getWindows();
			for (const window of windows) {
				if (!window?.handle) continue;
				const info = await getWindowInfo(window.handle);
				for (const rule of rules) {
					rule.apply(info);
				}
			}
		};

		setWindowsTitleBar();

		const handle = subscribeWindowManaged(async (payload) => {
			setTimeout(async () => {
				const hwnd = payload?.data?.managedWindow?.handle;
				if (hwnd) {
					const info = await getWindowInfo(hwnd);
					for (const rule of rules) {
						rule.apply(info);
					}
				}
			}, 200);
		});

		return () => {
			handle.close();
		};
	}, [rules]);

	return <></>;
};

export default TitleService;
