type LogoProps = {
    className?: string;
  };

export default function Logo({className}: LogoProps) {
    return (
        <h1 className="text-5xl px-6 pt-4 hindi">Hastakriti<span className={`text-[#6dd1c5] ${className}`}>.</span></h1>
    );
}