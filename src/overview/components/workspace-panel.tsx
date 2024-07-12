import type { Workspace } from "../../ipc/command";
import type { Optional } from "../../ipc/utils";
import SplitView from "./split-view";
import WindowView from "./window-view";

const WorkspacePanel = (props: { workspace: Optional<Workspace> }) => {
	const children = props.workspace?.children ?? [];
	const isHorizontal = props.workspace?.tilingDirection === "horizontal";
	const isFocus = props.workspace.focusIndex === 0;

	return (
		<div
			className={`w-full h-full rounded-2xl flex box-border p-4 gap-2 ${isFocus ? "bg-gray-7" : "bg-gray-8"} ${isHorizontal ? "flex-row" : "flex-col"}`}
		>
			{children.map((child) => {
				if (!child.sizePercentage) return;
				switch (child.type) {
					case "window":
						return <WindowView key={JSON.stringify(child)} window={child} />;
					case "split":
						return <SplitView key={JSON.stringify(child)} split={child} />;
				}
			})}
		</div>
	);
};

export default WorkspacePanel;
