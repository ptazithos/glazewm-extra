import React, { useEffect, useLayoutEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { exit } from "@tauri-apps/api/process";
import TitleService from "./services/titile";
import type { AppConfig } from "../ipc/utils";
import AlphaService from "./services/alpha";
import { getWindows } from "../ipc/command";

const DaemonApp = () => {
	const [appConfig, setAppConfig] = useState<AppConfig | null>(null);

	useEffect(() => {
		const init = async () => {
			const appConfig = await invoke<AppConfig>("get_app_config");
			setAppConfig(appConfig);

			await listen("clean_quit", async () => {
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
				}

				await exit(1);
			});
		};

		init();
	}, []);

	return appConfig ? (
		<>
			<TitleService config={appConfig} />
			<AlphaService config={appConfig} />
		</>
	) : (
		<></>
	);
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
