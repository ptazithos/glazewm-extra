import React, { useEffect, useLayoutEffect } from "react";
import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/tauri";
import { info } from "tauri-plugin-log-api";

import { subscribeFocusChanged } from "../ipc/subscribe";
import { getWindows } from "../ipc/command";
import type { AppConfig } from "../ipc/utils";

const DaemonApp = () => {
	useLayoutEffect(() => {
		const handle = subscribeFocusChanged(async (payload) => {
			const appConfig = await invoke<AppConfig>("get_app_config");

			if (!appConfig.translucent_window?.enable) {
				return;
			}

			const focused = payload?.data?.focusedContainer?.handle;

			const windows = await getWindows();
			for (const window of windows) {
				const hwnd = window?.handle;
				if (hwnd === focused) {
					invoke("set_window_alpha", { rawHandle: hwnd, alpha: 255 });
				} else {
					invoke("set_window_alpha", {
						rawHandle: hwnd,
						alpha: appConfig.translucent_window?.alpha ?? 240,
					});
				}
			}
		});

		return () => {
			handle.close();
		};
	}, []);

	return <></>;
};

const rootEl = document.getElementById("root");
if (rootEl) {
	const root = ReactDOM.createRoot(rootEl);
	root.render(
		<React.StrictMode>
			<DaemonApp />
		</React.StrictMode>,
	);
}
