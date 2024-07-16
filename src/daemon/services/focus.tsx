import { invoke } from "@tauri-apps/api";
import {
	type FocusChangedPayload,
	subscribeFocusChanged,
} from "../../ipc/subscribe";
import { useEffect } from "react";
import type { AppConfig } from "../../native";
import { getWindows } from "../../ipc/command";
import { getWindowInfo } from "../../native";
import type { Optional } from "../../utils";

const FocusService = (props: { config: AppConfig }) => {
	const focusedRules = props.config.focusedWindowsRules;
	const unfocusedRules = props.config.unfocusedWindowsRules;

	useEffect(() => {
		const setWindowStyle = async (payload: Optional<FocusChangedPayload>) => {
			const focused = payload?.data?.focusedContainer?.handle;

			const windows = await getWindows();
			for (const window of windows) {
				const hwnd = window?.handle;
				if (hwnd) {
					if (hwnd === focused) {
						const info = await getWindowInfo(hwnd);
						for (const rule of focusedRules) {
							rule.apply(info);
						}
					} else {
						const info = await getWindowInfo(hwnd);
						for (const rule of unfocusedRules) {
							rule.apply(info);
						}
					}
				}
			}
		};

		setWindowStyle({});

		const handle = subscribeFocusChanged(setWindowStyle);

		return () => {
			handle.close();
		};
	}, [focusedRules, unfocusedRules]);

	return <></>;
};

export default FocusService;
