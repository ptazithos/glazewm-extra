import { useEffect, useLayoutEffect, useRef, useState } from "react";
import type { WindowSpace } from "../../ipc/command";
import type { Optional } from "../../ipc/utils";
import { invoke } from "@tauri-apps/api";

const WindowView = (props: { window: Optional<WindowSpace> }) => {
	const containerRef = useRef<HTMLDivElement>(null);
	const textRef = useRef<HTMLSpanElement>(null);

	const [text, setText] = useState("");

	// const [showText, setShowText] = useState(false);

	// useLayoutEffect(() => {
	// 	const textWidth = text.current?.offsetWidth ?? 0;
	// 	const containerWidth = container.current?.offsetWidth ?? 0;

	// 	setShowText(containerWidth >= textWidth);
	// }, []);

	const textWidth = textRef.current?.offsetWidth;
	const containerWidth = containerRef.current?.offsetWidth;

	const showText =
		textWidth !== undefined &&
		containerWidth !== undefined &&
		containerWidth >= textWidth;

	useLayoutEffect(() => {
		(async () => {
			const text = (await invoke("get_window_name", {
				rawHandle: props.window.handle,
			})) as string;
			setText(text.split("-").at(-1) ?? "");
		})();
	}, [props]);

	return (
		<div
			ref={containerRef}
			className="flex-1 border-gray border-solid border-2 rounded-xl flex justify-center items-center"
		>
			<span ref={textRef} className="absolute text-gray max-w-full">
				{showText ? text : "..."}
			</span>
		</div>
	);
};

export default WindowView;
