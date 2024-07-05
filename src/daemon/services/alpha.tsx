import { invoke } from "@tauri-apps/api";
import { subscribeFocusChanged } from "../../ipc/subscribe";
import { useEffect } from "react";
import type { AppConfig } from "../../ipc/utils";
import { getWindows } from "../../ipc/command";

const AlphaService = (props: { config: AppConfig }) => {
	const alphaConfig = props.config.translucent_window;

	useEffect(() => {
		const setWindowsAlpha = async (alpha: number) => {
			const windows = await getWindows();
			for (const window of windows) {
				const hwnd = window?.handle;

				hwnd &&
					invoke("set_window_alpha", {
						rawHandle: hwnd,
						alpha,
					});
			}
		};

		if (!alphaConfig?.enable) return;

		const handle = subscribeFocusChanged(async (payload) => {
			const focused = payload?.data?.focusedContainer?.handle;
			await setWindowsAlpha(alphaConfig.alpha ?? 240);
			await invoke("set_window_alpha", { rawHandle: focused, alpha: 255 });
		});

		return () => {
			handle.close();
		};
	}, [alphaConfig]);

	return <></>;
};

export default AlphaService;
