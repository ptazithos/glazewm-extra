import React, { useEffect, useLayoutEffect, useState } from "react";
import ReactDOM from "react-dom/client";

import ManageService from "./services/manage";
import FocusService from "./services/focus";
import CleanUpService from "./services/cleanup";

import { type AppConfig, getAppConfig } from "../native";

const DaemonApp = () => {
	const [appConfig, setAppConfig] = useState<AppConfig | null>(null);

	useEffect(() => {
		const init = async () => {
			const appConfig = await getAppConfig();
			setAppConfig(appConfig);
		};

		init();
	}, []);

	return appConfig ? (
		<>
			<ManageService config={appConfig} />
			<FocusService config={appConfig} />
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
