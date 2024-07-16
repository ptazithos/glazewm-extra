import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import { LogicalSize, WebviewWindow } from "@tauri-apps/api/window";
import { info } from "tauri-plugin-log-api";

import { getWorkspaces, type Workspace } from "../ipc/command";
import type { Optional } from "../utils";
import WorkspacePanel from "./components/workspace-panel";

import "./index.css";
import {
	subscribeAll,
	subscribeFocusChanged,
	subscribeWindowManaged,
	subscribeWindowUnmanaged,
} from "../ipc/subscribe";
import { listen } from "@tauri-apps/api/event";

const OverviewApp = () => {
	const refWindow = useRef(WebviewWindow.getByLabel("overview"));
	const show = useRef(false);
	const [workspaces, setWorkspaces] = useState<Optional<Array<Workspace>>>([]);

	useEffect(() => {
		const updateWorkspaces = async () => {
			const window = refWindow.current;
			const workspaces = await getWorkspaces();

			await window?.setSize(new LogicalSize(workspaces.length * 280, 180));
			await window?.center();

			setWorkspaces(workspaces);
		};

		updateWorkspaces();

		subscribeAll(() => {
			updateWorkspaces();
		});

		listen("trigger-overview", () => {
			if (show.current) {
				refWindow.current?.hide();
			} else {
				refWindow.current?.show();
			}

			show.current = !show.current;
		});
	}, []);

	console.log(workspaces);

	return (
		<div className="h-100vh flex gap-5">
			{workspaces.map(
				(workspace) =>
					workspace && (
						<div className="flex-1" key={JSON.stringify(workspace)}>
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
