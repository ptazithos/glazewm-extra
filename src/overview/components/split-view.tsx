import type { SplitSpace } from "../../ipc/command";
import type { Optional } from "../../ipc/utils";
import WindowView from "./window-view";

const SplitView = (props: { split: Optional<SplitSpace> }) => {
	const children = props.split.children ?? [];
	const isHorizontal = props.split.tilingDirection === "horizontal";

	return (
		<div
			className={`flex-1 flex gap-2 ${isHorizontal ? "flex-row" : "flex-col"}`}
		>
			{children.map((child) => {
				if (!child?.sizePercentage) return <></>;
				switch (child.type) {
					case "window":
						return <WindowView window={child} key={JSON.stringify(child)} />;
					case "split":
						return <SplitView split={child} key={JSON.stringify(child)} />;
				}
			})}
		</div>
	);
};

export default SplitView;
