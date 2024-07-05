import { useEffect } from "react";
import type { AppConfig } from "../../ipc/utils";
import { invoke } from "@tauri-apps/api";
import { getWindows } from "../../ipc/command";
import { subscribeWindowManaged } from "../../ipc/subscribe";

const TitleService = (props: { config: AppConfig }) => {
	const tileBarConfig = props.config.title_bar;

	useEffect(() => {
		const setWindowsTitleBar = async (enable: boolean) => {
			const windows = await getWindows();
			for (const window of windows) {
				const hwnd = window?.handle;
				hwnd &&
					invoke("set_window_titlebar", { rawHandle: hwnd, titlebar: enable });
			}
		};

		setWindowsTitleBar(!(tileBarConfig?.enable ?? true));

		if (!tileBarConfig?.enable) return;

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
	}, [tileBarConfig]);

	return <></>;
};

export default TitleService;
