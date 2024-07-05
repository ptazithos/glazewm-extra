import type { Optional } from "./utils";
import { GLAZEWM_IPC_ADDR } from "./utils";

type Command = "windows";

export const command = <T>(command: Command): Promise<Optional<T>> => {
	return new Promise((resolve, reject) => {
		const ws = new WebSocket(GLAZEWM_IPC_ADDR);

		ws.onopen = () => {
			ws.send(command);
		};

		ws.onmessage = (event) => {
			resolve(JSON.parse(event.data) as Optional<T>);
			ws.close();
		};

		ws.onerror = () => {
			reject();
			ws.close();
		};
	});
};

type Window = {
	handle: number;
};
type WindowsPayload = { data: Array<Window> };

export const getWindows = async () =>
	(await command<WindowsPayload>("windows")).data ?? [];
