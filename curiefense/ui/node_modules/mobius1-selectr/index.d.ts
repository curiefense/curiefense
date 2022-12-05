export interface IData {
	text: string;
	value: string;
	selected?: boolean;
	disabled?: boolean;
	children?: IData[];
}
export interface IOptions {
	defaultSelected?: boolean;
	multiple?: boolean;
	searchable?: boolean;
	allowDeselect?: boolean;
	clearable?: boolean;
	width?: number | string;
	placeholder?: string;
	maxSelections?: number;
	taggable?: boolean;
	tagSeperators?: string[];
	tagPlaceholder?: string;
	data?: IData[];
	renderOption?(): string;
	renderSelection?(): string;
	pagination?: number;
	nativeDropdown?: boolean;
	closeOnScroll?: boolean;
	sortSelected?: 'text' | 'value';
	customClass?: string;
	messages?: {
		noResults?: string;
		noOptions?: string;
		maxSelections?: string;
		tagDuplicate?: string;
		searchPlaceholder?: string;
	};
}

export default class Selectr {
	constructor(select: HTMLSelectElement, options: IOptions);
	public setValue(value: string | string[]): void;
	public getValue(toObject?: boolean, toJson?: boolean): string | { text: string, value: string };
	public search(query: string, anchor?: boolean): IData[];
	public add(data: IData | IData[]): void;
	public remove(data: number | string | number[] | string[]): void;
	public removeAll(): void;
	public serialize(toJson?: boolean): string | IData[];
	public open(): void;
	public close(): void;
	public toggle(): void;
	public clear(): void;
	public reset(): void;
	public disable(): void;
	public enable(): void;
	public on(event: 'selectr.init', fun: () => void): void;
	public on(event: 'selectr.select', fun: (option: HTMLOptionElement) => void): void;
	public on(event: 'selectr.deselect', fun: (option: HTMLOptionElement) => void): void;
	public on(event: 'selectr.change', fun: (option: HTMLOptionElement) => void): void;
	public on(event: 'selectr.open', fun: () => void): void;
	public on(event: 'selectr.close', fun: () => void): void;
	public on(event: 'selectr.clear', fun: () => void): void;
	public on(event: 'selectr.reset', fun: () => void): void;
	public on(event: 'selectr.paginate', fun: (data: {
		items: number;
		total: number;
		page: number;
		pages: number;
	}) => void): void;
}

