import type { WindowSpace } from "../../ipc/command";
import type { Optional } from "../../ipc/utils";

const WindowView = (props: { window: Optional<WindowSpace> }) => {
	return (
		<div className="flex-1 border-gray border-solid border-2 rounded-xl flex justify-center items-center">
			<span className="text-gray">Window</span>
		</div>
	);
};

export default WindowView;
