import { useEffect, useLayoutEffect, useRef, useState } from "react";
import type { WindowSpace } from "../../ipc/command";
import type { Optional } from "../../ipc/utils";

const WindowView = (props: { window: Optional<WindowSpace> }) => {
	const container = useRef<HTMLDivElement>(null);
	const text = useRef<HTMLSpanElement>(null);
	const [showText, setShowText] = useState(false);

	useLayoutEffect(() => {
		const textWidth = text.current?.offsetWidth ?? 0;
		const containerWidth = container.current?.offsetWidth ?? 0;

		setShowText(containerWidth >= textWidth);
	}, []);

	return (
		<div
			ref={container}
			className="flex-1 border-gray border-solid border-2 rounded-xl flex justify-center items-center"
		>
			<span
				ref={text}
				className={`absolute text-gray ${showText ? "" : "invisible"}`}
			>
				Windows
			</span>
		</div>
	);
};

export default WindowView;
