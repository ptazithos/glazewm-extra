import { useEffect, useLayoutEffect, useRef, useState } from "react";
import type { WindowSpace } from "../../ipc/command";
import type { Optional } from "../../ipc/utils";
import { invoke } from "@tauri-apps/api";

const WindowView = (props: { window: Optional<WindowSpace> }) => {
	const containerRef = useRef<HTMLDivElement>(null);
	const textRef = useRef<HTMLDivElement>(null);

	const [text, setText] = useState("");

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
			className="flex-1 border-gray border-solid border-2 rounded-xl flex justify-center items-center relative"
		>
			<div
				ref={textRef}
				className="absolute text-gray max-w-full break-words text-align-center"
			>
				{text}
			</div>
		</div>
	);
};

export default WindowView;
