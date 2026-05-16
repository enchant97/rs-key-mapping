# Change Log

## 0.7.1
- add missing defmt support for `Keys`

## 0.7.0
- add KeyStatusUpdate conversion from embassy-usb-host
- add defmt support

## 0.6.0
- Ability to convert from other KeyboardReports

## 0.5.0
- Rename `KeyboardAction` to `KeyboardReport`
- `KeyboardReport` can now store a flexible amount of keys
- Add support for converting from `embassy_usb_host::class::hid::KeyboardReport`
- Add key type to key mappings
- bump deps
