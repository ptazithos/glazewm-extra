import { useEffect } from "react";
import { getWindows } from "../../ipc/command";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { exit } from "@tauri-apps/api/process";

const CleanUpService = () => {
	useEffect(() => {
		listen("clean_quit", async () => {
			const windows = await getWindows();
			for (const window of windows) {
				await invoke("set_window_alpha", {
					rawHandle: window.handle,
					alpha: 255,
				});

				await invoke("set_window_titlebar", {
					rawHandle: window.handle,
					titlebar: true,
				});

				await invoke("set_window_rounded", {
					rawHandle: window.handle,
					rounded: true,
				});
			}

			await exit(1);
		});
	}, []);
	return <></>;
};

export default CleanUpService;
