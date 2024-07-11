import type { Workspace } from "../../ipc/command";
import type { Optional } from "../../ipc/utils";
import SplitView from "./split-view";
import WindowView from "./window-view";

const WorkspacePanel = (props: { workspace: Optional<Workspace> }) => {
	const children = props.workspace?.children ?? [];
	const isHorizontal = props.workspace?.tilingDirection === "horizontal";

	return (
		<div
			className={`w-full h-full bg-gray-8 rounded-2xl flex box-border p-4 gap-2 ${isHorizontal ? "flex-row" : "flex-col"}`}
		>
			{children.map((child) => {
				if (!child?.sizePercentage) return <></>;
				switch (child.type) {
					case "window":
						return <WindowView window={child} />;
					case "split":
						return <SplitView split={child} />;
				}
			})}
		</div>
	);
};

export default WorkspacePanel;
