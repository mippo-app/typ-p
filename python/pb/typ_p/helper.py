
import datetime
import numpy as np

y_mask = np.uint32(~np.uint16(0)) << 16
m_mask = np.uint32(~np.uint16(0)) << 8
d_mask = np.uint32(~np.uint16(0))


def value_to_date(v):
  y = (v & y_mask) >> 16
  m = (v & m_mask) >> 8
  d = (v & d_mask)

  return datetime.date(int(np.uint16(y)), int(np.uint8(m)), int(np.uint8(d)))


if __name__ == "__main__":
  print(y_mask)
  print(m_mask)
  print(d_mask)

  v = 131662348

  y = (v & y_mask) >> 16
  m = (v & m_mask) >> 8
  d = (v & d_mask)

  print(y)
  print(m)
  print(d)
