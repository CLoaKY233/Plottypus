import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

type SelectBoxProps = {
  label?: string;
  placeholder?: string;
  options?: string[] | Number[];
  className?: string;
};

export function SelectBox({ options=[], label="", placeholder="Select an option", className="" }: SelectBoxProps) {
  return (
    <Select>
      <SelectTrigger className={`w-[180px] ${className}`}>
        <SelectValue placeholder={placeholder} />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          {label!='' && (<SelectLabel className="font-semibold">{label}</SelectLabel>)}
          {options.map((option) => (
            <SelectItem key={`${option}`} value={`${option}`}>{`${option}`}</SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
  )
}