import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import { register, unregister } from "@tauri-apps/api/globalShortcut";
import { LogicalSize, WebviewWindow } from "@tauri-apps/api/window";
import { info } from "tauri-plugin-log-api";

import { getWorkspace, type Workspace } from "../ipc/command";
import type { Optional } from "../ipc/utils";
import WorkspacePanel from "./components/workspace-panel";

import "./index.css";

const OverviewApp = () => {
	const refShow = useRef(false);
	const refWindow = useRef(WebviewWindow.getByLabel("overview"));

	const [workspaces, setWorkspaces] = useState<Optional<Array<Workspace>>>([]);

	useEffect(() => {
		(async () => {
			const window = refWindow.current;
			const workspaces = await getWorkspace();
			info(JSON.stringify(workspaces));

			window?.setSize(new LogicalSize(workspaces.length * 280, 180));
			window?.center();

			setWorkspaces(workspaces);
		})();
	}, []);

	return (
		<div className="h-100vh flex gap-5">
			{workspaces.map(
				(workspace, index) =>
					workspace && (
						<div className="flex-1  " key={JSON.stringify(workspace)}>
							<WorkspacePanel workspace={workspace} />
						</div>
					),
			)}
		</div>
	);
};

const rootEl = document.getElementById("root");
if (rootEl) {
	const root = ReactDOM.createRoot(rootEl);
	root.render(
		<React.StrictMode>
			<OverviewApp />
		</React.StrictMode>,
	);
}
