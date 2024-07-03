import type { Optional } from "./utils";
import { GLAZEWM_IPC_ADDR } from "./utils";

export type Event = "focus_changed";

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
