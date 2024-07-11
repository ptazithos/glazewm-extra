import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import { register } from "@tauri-apps/api/globalShortcut";
import { LogicalSize, WebviewWindow } from "@tauri-apps/api/window";
import { info } from "tauri-plugin-log-api";

import { getWorkspace, type Workspace } from "../ipc/command";
import type { Optional } from "../ipc/utils";
import WorkspacePanel from "./components/workspace-panel";

const OverviewApp = () => {
	const refShow = useRef(false);
	const refWindow = useRef(WebviewWindow.getByLabel("overview"));

	const [workspaces, setWorkspaces] = useState<Optional<Array<Workspace>>>([]);

	useEffect(() => {
		register("Alt+Shift+O", async () => {
			info("Alt+Shift+O was pressed");
			const window = refWindow.current;
			const workspaces = await getWorkspace();
			setWorkspaces(await getWorkspace());

			if (refShow.current) {
				window?.hide();
			} else {
				window?.setSize(new LogicalSize(workspaces.length * 320, 180));
				window?.show();
			}
			refShow.current = !refShow.current;
		});
	}, []);

	return (
		<>
			{workspaces.map((workspace) => (
				<WorkspacePanel key={JSON.stringify(workspace)} />
			))}
		</>
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
