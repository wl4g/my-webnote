type Size = 'small' | 'middle' | 'large';

interface Props {
  className?: string;
  size?: Size;
  url?: string;
  onClick?: () => void;
}

const getTextSizeName = (size: Size) => {
  switch (size) {
    case 'small':
      return 'text-xl';
    case 'middle':
      return 'text-3xl';
    case 'large':
      return 'text-5xl';
  }
};

export default function Logo({ size = 'small', className = '', url, onClick }: Props) {
  return (
    <div
      className={`flex items-center text-xl font-mono cursor-pointer ${getTextSizeName(
        size
      )} ${className}`}
      onClick={() => {
        onClick?.();
        url && window.open(url);
      }}
    >
      <span
        className={`tracking-wider bg-clip-text text-transparent text-sky-500 bg-gradient-to-r 
      from-sky-300 to-sky-600 decoration-cyan-100 underline-offset-2  underline font-semibold 
      ${getTextSizeName(size)}`}
      >
        Mywebnote
      </span>
    </div>
  );
}
