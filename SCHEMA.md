# Schema

## Event types

```
MOTION_EVENT_TYPE = 0;
SCROLL_EVENT_TYPE = 1;
TOUCH_BEGIN_EVENT_TYPE = 2;
TOUCH_UPDATE_EVENT_TYPE = 3;
TOUCH_END_EVENT_TYPE = 4;
BUTTON_PRESS_EVENT_TYPE = 5;
BUTTON_RELEASE_EVENT_TYPE = 6;
METADATA_CHANGED_EVENT_TYPE = 7;
```

## Version `20220519T201520Z`

```
{
  MOTION_EVENT_TYPE: {
    types: [
      'type:type',
      't:timestamp:ms',
      'xIntegral:integer',
      'xFraction:integer',
      'yIntegral:integer',
      'yFraction:integer',
      'rootX:integer',
      'rootY:integer',
    ],
    name: 'XinputRawMotion',
    description: 'Raw motion event',
  },

 SCROLL_EVENT_TYPE: {
    types: [
      'type:type',
      't:timestamp:ms',
      'valueIntegral:integer',
      'valueFraction:integer',
      'rootX:integer',
      'rootY:integer',
    ],
    name: 'XinputRawMotion',
    description: 'Scroll event',
  },

  TOUCH_BEGIN_EVENT_TYPE: {
    types: [
      'type:type',
      't:timestamp:ms',
      'xIntegral:integer',
      'xFraction:integer',
      'yIntegral:integer',
      'yFraction:integer',
      'rootX:integer',
      'rootY:integer',
    ],
    name: 'XinputRawTouchBegin',
    description: 'Raw touch begin event',
  },

  TOUCH_UPDATE_EVENT_TYPE: {
    types: [
      'type:type',
      't:timestamp:ms',
      'xIntegral:integer',
      'xFraction:integer',
      'yIntegral:integer',
      'yFraction:integer',
      'rootX:integer',
      'rootY:integer',
    ],
    name: 'XinputRawTouchUpdate',
    description: 'Raw touch update event',
  },

  TOUCH_END_EVENT_TYPE: {
    types: [
      'type:type',
      't:timestamp:ms',
      'xIntegral:integer',
      'xFraction:integer',
      'yIntegral:integer',
      'yFraction:integer',
      'rootX:integer',
      'rootY:integer',
    ],
    name: 'XinputRawTouchEnd',
    description: 'Raw touch end event',
  },

  BUTTON_PRESS_EVENT_TYPE: {
    types: [
      'type:type',
      't:timestamp:ms',
      'rootX:integer',
      'rootY:integer',
      'detail:integer',
    ],
    name: 'XinputRawButtonPress',
    description: 'Raw button press event',
  },

  BUTTON_RELEASE_EVENT_TYPE: {
    types: [
      'type:type',
      't:timestamp:ms',
      'rootX:integer',
      'rootY:integer',
      'detail:integer',
    ],
    name: 'XinputRawButtonRelease',
    description: 'Raw button release event',
  },

  METADATA_CHANGED_EVENT_TYPE: {
    types: [
      'type:type',
      't:timestamp:ms',
      'metadata:object',
    ],
    name: 'MetadataChangedEvent',
    description: 'Metadata changed event',
  },
}
