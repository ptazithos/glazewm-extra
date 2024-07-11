import type { Optional } from "./utils";
import { GLAZEWM_IPC_ADDR } from "./utils";

export type Event = "focus_changed" | "window_managed" | "window_unmanaged";

export const subscribe = <T>(
	event: Event,
	onMessage: (payload: Optional<T>) => void,
) => {
	const ws = new WebSocket(GLAZEWM_IPC_ADDR);

	ws.onmessage = (event) => {
		const payload = JSON.parse(event.data);
		const isSubscription = payload?.messageType === "event_subscription";
		isSubscription && onMessage(payload);
	};

	ws.onopen = () => {
		ws.send(`subscribe -e ${event}`);
	};

	return ws;
};

export type FocusChangedPayload = {
	data: {
		focusedContainer: {
			handle: number;
		};
	};
};

export const subscribeFocusChanged = (
	onMessage: (payload: Optional<FocusChangedPayload>) => void,
) => {
	return subscribe<FocusChangedPayload>("focus_changed", onMessage);
};

export type WindowManagedPayload = {
	data: {
		managedWindow: {
			handle: number;
		};
	};
};

export const subscribeWindowManaged = (
	onMessage: (payload: Optional<WindowManagedPayload>) => void,
) => {
	return subscribe("window_managed", onMessage);
};

export const subscribeWindowUnmanaged = (
	onMessage: (payload: Optional<WindowManagedPayload>) => void,
) => {
	return subscribe("window_unmanaged", onMessage);
};
