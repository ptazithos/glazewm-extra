import React, { useEffect, useLayoutEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/tauri";

import TitleService from "./services/titile";
import AlphaService from "./services/alpha";
import CleanUpService from "./services/cleanup";

import type { AppConfig } from "../ipc/utils";

const DaemonApp = () => {
	const [appConfig, setAppConfig] = useState<AppConfig | null>(null);

	useEffect(() => {
		const init = async () => {
			const appConfig = await invoke<AppConfig>("get_app_config");
			setAppConfig(appConfig);
		};

		init();
	}, []);

	return appConfig ? (
		<>
			<TitleService config={appConfig} />
			<AlphaService config={appConfig} />
			<CleanUpService />
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
