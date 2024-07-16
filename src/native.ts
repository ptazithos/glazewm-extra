import { invoke } from "@tauri-apps/api";

export const getWindowInfo = async (rawHandle: number) => {
	const [_title, _className, _processName] = await Promise.all([
		invoke<string | null>("get_window_title", { rawHandle }),
		invoke<string | null>("get_window_class", { rawHandle }),
		invoke<string | null>("get_window_process_name", { rawHandle }),
	]);

	const title = _title ?? "";
	const className = _className ?? "";
	const processName = (_processName ?? "").split("\\").at(-1) ?? "";

	return {
		title,
		className,
		processName,
	};
};
