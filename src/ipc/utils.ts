export type Optional<T> = {
	[P in keyof T]?: T[P] extends (infer U)[]
		? Optional<U>[]
		: T[P] extends object | undefined
			? Optional<T[P]>
			: T[P];
};

export type AppConfig = Optional<{
	translucent_window: {
		enable: boolean;
		alpha: number;
	};
}>;

export const GLAZEWM_IPC_ADDR = "ws://localhost:6123";
