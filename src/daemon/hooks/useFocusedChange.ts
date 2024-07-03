import { useEffect } from "react";
import { info } from "tauri-plugin-log-api";

import { subscribe } from "../../network/subscribe";

export const useFocusedChange = () => {
	useEffect(() => {
		subscribe("focus_changed", (payload) => {
			const hwnd = payload?.data?.focusedContainer?.handle;
		});
	}, []);
};
