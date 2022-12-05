export function toast(options: Options): void;

export function setDefaults(options: Options): void;

export function resetDefaults(): void;

export interface Options {
  message?: string | HTMLElement;
  type?: ToastType;
  duration?: number;
  position?: ToastPosition;
  dismissible?: boolean;
  pauseOnHover?: boolean;
  closeOnClick?: boolean;
  opacity?: number;
  animate?: ToastAnimation;
  appendTo?: Node;
  extraClasses?: string;
}

export type ToastType = 'is-primary'
  | 'is-link'
  | 'is-info'
  | 'is-success'
  | 'is-warning'
  | 'is-danger'
  | 'is-white'
  | 'is-black'
  | 'is-light'
  | 'is-dark';

export type ToastPosition = 'top-left'
  | 'top-right'
  | 'top-center'
  | 'center'
  | 'bottom-left'
  | 'bottom-center'
  | 'bottom-right';

export interface ToastAnimation {
  in?: string;
  out?: string;
}

export function setDoc(newDoc: HTMLElement): void;
