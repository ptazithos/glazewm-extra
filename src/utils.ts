export type Optional<T> = {
	[P in keyof T]?: T[P] extends (infer U)[]
		? Optional<U>[]
		: T[P] extends object | undefined
			? Optional<T[P]>
			: T[P];
};
